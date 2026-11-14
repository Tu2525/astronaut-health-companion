import { OrbitControls, useGLTF } from "@react-three/drei";
import { Canvas, useFrame, useThree } from "@react-three/fiber";
import { Suspense, useEffect, useMemo, useRef } from "react";
import type { ElementRef } from "react";
import * as THREE from "three";
import type { Region } from "../../../contract/Region";
import type { Status } from "../../../contract/Status";

export interface BodyCanvasProps {
  status: Record<Region, Status>;
  lit: Set<Region> | null;
  selected: Region | null;
  focus: { region: Region; n: number } | null;
  onPick(region: Region): void;
  onEmpty(): void;
  onReady(): void;
}

const MODEL_URL = `${import.meta.env.BASE_URL}models/body.glb`;

/** Resolve any CSS colour expression (tokens, color-mix) to a three.js colour at runtime. */
function cssColor(expr: string) {
  const el = document.createElement("span");
  el.style.color = expr;
  document.body.append(el);
  const resolved = getComputedStyle(el).color;
  el.remove();
  const ctx = document.createElement("canvas").getContext("2d", { willReadFrequently: true })!;
  ctx.fillStyle = resolved;
  ctx.fillRect(0, 0, 1, 1);
  const [r, g, b] = ctx.getImageData(0, 0, 1, 1).data;
  return new THREE.Color().setRGB(r / 255, g / 255, b / 255, THREE.SRGBColorSpace);
}

// X-ray material: lambert core + fresnel rim. One shader, per-region uniforms.
const VS = `varying vec3 vN;varying vec3 vV;varying float vY;void main(){vec4 mv=modelViewMatrix*vec4(position,1.);vY=(modelMatrix*vec4(position,1.)).y;vN=normalize(normalMatrix*normal);vV=normalize(-mv.xyz);gl_Position=projectionMatrix*mv;}`;
const FS = `uniform vec3 uC;uniform float uBody,uRim,uA,uRimA,uGlow,uSel,uHov,uT,uY0,uY1;varying vec3 vN;varying vec3 vV;varying float vY;
void main(){vec3 n=normalize(vN);if(!gl_FrontFacing)n=-n;float f=pow(1.-abs(dot(n,vV)),2.2);
float l=.35+.65*max(dot(n,normalize(vec3(.4,.7,.6))),0.);
float yy=(vY-uY0)/max(uY1-uY0,1e-4);float band=exp(-pow((yy-fract(uT*.45)*1.4+.2)*9.,2.));
vec3 c=uC*(uBody*l+uGlow)+uC*f*uRim;
c+=uC*(uHov*.3+uSel*(.35+f*1.6+band*1.1))+vec3(1.)*uSel*band*.35;
gl_FragColor=vec4(c,clamp(uA+f*uRimA+uSel*(.12+f*.35+band*.2)+uHov*.08,0.,1.));}`;

interface Look { body: number; rim: number; a: number; rimA: number; order: number; shell?: boolean }
const LOOK: Record<string, Look> = {
  skin: { body: 0, rim: 1.1, a: 0, rimA: 0.32, order: 6, shell: true },
  muscles: { body: 0.15, rim: 0.7, a: 0.015, rimA: 0.12, order: 5, shell: true },
  bones: { body: 0.55, rim: 0.6, a: 0.08, rimA: 0.3, order: 1, shell: true },
  head: { body: 0.55, rim: 0.6, a: 0.08, rimA: 0.32, order: 1, shell: true },
};
const ORGAN: Look = { body: 0.9, rim: 0.8, a: 0.82, rimA: 0.18, order: 3 };
const PRIORITY = ["eyes", "heart", "lungs", "mind", "immune", "gut", "head", "bones", "muscles", "skin"];
const keyOf = (name: string) => name.replace(/\.\d+$/, ""); // Blender suffixes duplicates: heart.001

interface RegionState {
  mesh: THREE.Mesh; look: Look; uniforms: Record<string, THREE.IUniform>;
  fade: number; sel: number; hov: number; color: THREE.Color; center: THREE.Vector3; size: number;
}

function Body(props: BodyCanvasProps) {
  const { scene } = useGLTF(MODEL_URL, false, false);
  const { camera, gl, size } = useThree();
  const controls = useRef<ElementRef<typeof OrbitControls>>(null);
  const live = useRef(props);
  live.current = props;
  const reduce = useMemo(() => matchMedia("(prefers-reduced-motion: reduce)").matches, []);

  const palette = useMemo(() => {
    const neutral = cssColor("color-mix(in srgb, var(--text-muted) 88%, var(--text))");
    const status = Object.fromEntries((["nominal", "watch", "caution", "urgent", "unknown"] as const)
      .map((s) => [s, cssColor(`var(--status-${s})`)])) as Record<Status, THREE.Color>;
    return {
      status, neutral,
      calmShell: neutral.clone().lerp(status.nominal, 0.06),
      calmOrgan: neutral.clone().lerp(status.nominal, 0.22),
    };
  }, []);

  const world = useMemo(() => {
    const root = scene.clone(true);
    const regions: Record<string, RegionState> = {};
    root.traverse((o) => {
      const mesh = o as THREE.Mesh;
      if (!mesh.isMesh) return;
      const key = keyOf(mesh.name);
      const look = LOOK[key] ?? ORGAN;
      const uniforms = {
        uC: { value: new THREE.Color() }, uBody: { value: look.body }, uRim: { value: look.rim },
        uA: { value: look.a }, uRimA: { value: look.rimA }, uGlow: { value: 0 }, uSel: { value: 0 },
        uHov: { value: 0 }, uT: { value: 0 }, uY0: { value: 0 }, uY1: { value: 1 },
      };
      mesh.material = new THREE.ShaderMaterial({
        vertexShader: VS, fragmentShader: FS, transparent: true, depthWrite: !look.shell, uniforms,
        side: look.shell ? THREE.DoubleSide : THREE.FrontSide,
        blending: look.shell ? THREE.AdditiveBlending : THREE.NormalBlending,
      });
      mesh.renderOrder = look.order;
      regions[key] = { mesh, look, uniforms, fade: 1, sel: 0, hov: 0, color: palette.neutral.clone(), center: new THREE.Vector3(), size: 1 };
    });
    // Centre on the spine (head), not the bounding box: arms make the box lopsided.
    const box = new THREE.Box3().setFromObject(root);
    const c = box.getCenter(new THREE.Vector3());
    if (regions.head) {
      const spine = new THREE.Box3().setFromObject(regions.head.mesh).getCenter(new THREE.Vector3());
      c.x = spine.x;
      c.z = spine.z;
    }
    root.position.sub(c);
    root.updateMatrixWorld(true);
    const bodySize = box.getSize(new THREE.Vector3());
    for (const R of Object.values(regions)) {
      const b = new THREE.Box3().setFromObject(R.mesh);
      R.center = b.getCenter(new THREE.Vector3());
      R.size = b.getSize(new THREE.Vector3()).length();
      R.uniforms.uY0.value = b.min.y;
      R.uniforms.uY1.value = b.max.y;
    }
    return { root, regions, bodySize };
  }, [scene, palette]);

  // Camera: critically damped approach to a goal (target + distance). Home fits the body height only:
  // the model's box is far wider than the visible body, a width fit shrank it on phones.
  const home = useRef({ dist: 5, placed: false });
  const goal = useRef({ target: new THREE.Vector3(), dist: 5 });
  const focused = useRef(false);
  const st = useRef({ moved: false, dragging: false, hovered: null as string | null, pressed: null as string | null });
  const hovEv = useRef<PointerEvent | null>(null);
  const pickRef = useRef<(e: PointerEvent) => string | null>(() => null);
  const tmp = useMemo(() => ({ dir: new THREE.Vector3(), off: new THREE.Vector3(), sp: new THREE.Spherical() }), []);

  useEffect(() => {
    const t = Math.tan(THREE.MathUtils.degToRad((camera as THREE.PerspectiveCamera).fov / 2));
    home.current.dist = world.bodySize.y * 1.04 / 2 / t * (size.width / size.height < 0.8 ? 1.12 : 1);
    if (!home.current.placed) {
      home.current.placed = true;
      camera.position.set(0, 0, home.current.dist);
      goal.current.dist = home.current.dist;
    } else if (!focused.current) goal.current.dist = home.current.dist;
  }, [camera, size, world]);

  useEffect(() => {
    const f = props.focus ? world.regions[props.focus.region] : undefined;
    focused.current = !!f;
    goal.current = f
      ? { target: f.center.clone(), dist: Math.max(f.size * 1.7, home.current.dist * 0.32) }
      : { target: new THREE.Vector3(), dist: home.current.dist };
  }, [props.focus, world]);

  useEffect(() => { live.current.onReady(); }, [world]);

  // Picking: tap vs drag with 6px hysteresis; press highlights on pointer-down, selection commits on release.
  useEffect(() => {
    const canvas = gl.domElement;
    const ray = new THREE.Raycaster();
    const ptr = new THREE.Vector2();
    const meshes = Object.values(world.regions).map((R) => R.mesh);
    let down: [number, number] | null = null;
    const pick = (e: PointerEvent) => {
      const r = canvas.getBoundingClientRect();
      ptr.set((e.clientX - r.left) / r.width * 2 - 1, -(e.clientY - r.top) / r.height * 2 + 1);
      ray.setFromCamera(ptr, camera);
      const hits = ray.intersectObjects(meshes, false).map((h) => keyOf(h.object.name));
      const k = PRIORITY.find((p) => hits.includes(p) && p !== "skin" && p !== "muscles") ?? hits[0];
      return k && k !== "skin" ? k : null;
    };
    pickRef.current = pick;
    const far = (e: PointerEvent) => !down || Math.hypot(e.clientX - down[0], e.clientY - down[1]) > 6;
    const onDown = (e: PointerEvent) => { down = [e.clientX, e.clientY]; st.current.pressed = pick(e); };
    const onMove = (e: PointerEvent) => {
      if (st.current.pressed && far(e)) st.current.pressed = null;
      if (e.pointerType === "mouse" && !e.buttons) hovEv.current = e;
    };
    const onUp = (e: PointerEvent) => {
      st.current.pressed = null;
      if (far(e)) return;
      const k = pick(e);
      if (k) live.current.onPick(k as Region); else live.current.onEmpty();
    };
    const onCancel = () => { st.current.pressed = null; };
    const onLeave = () => { st.current.hovered = null; canvas.style.cursor = ""; };
    canvas.addEventListener("pointerdown", onDown);
    canvas.addEventListener("pointermove", onMove);
    canvas.addEventListener("pointerup", onUp);
    canvas.addEventListener("pointercancel", onCancel);
    canvas.addEventListener("pointerleave", onLeave);
    return () => {
      canvas.removeEventListener("pointerdown", onDown);
      canvas.removeEventListener("pointermove", onMove);
      canvas.removeEventListener("pointerup", onUp);
      canvas.removeEventListener("pointercancel", onCancel);
      canvas.removeEventListener("pointerleave", onLeave);
    };
  }, [gl, camera, world]);

  useFrame((state, delta) => {
    const ctl = controls.current;
    if (!ctl) return;
    const dt = Math.min(delta, 0.05), t = state.clock.elapsedTime;
    const k = reduce ? 1 : 1 - Math.exp(-dt / 0.11);
    ctl.target.lerp(goal.current.target, k);
    tmp.dir.copy(camera.position).sub(ctl.target);
    const d = tmp.dir.length();
    camera.position.copy(ctl.target).addScaledVector(tmp.dir.normalize(), d + (goal.current.dist - d) * k);
    ctl.autoRotate = !reduce && !st.current.moved && !focused.current;
    // Rubber-band: hard limits are wide, the soft range springs back once the drag lets go.
    if (!st.current.dragging) {
      const p = ctl.getPolarAngle(), c = Math.min(Math.PI * 0.7, Math.max(Math.PI * 0.3, p));
      if (Math.abs(c - p) > 1e-4) {
        tmp.sp.setFromVector3(tmp.off.copy(camera.position).sub(ctl.target));
        tmp.sp.phi = p + (c - p) * (reduce ? 1 : 1 - Math.exp(-dt / 0.12));
        camera.position.copy(ctl.target).add(tmp.off.setFromSpherical(tmp.sp));
      }
    }
    if (hovEv.current) {
      st.current.hovered = pickRef.current(hovEv.current);
      gl.domElement.style.cursor = st.current.hovered ? "pointer" : "";
      hovEv.current = null;
    }
    const { status, lit, selected, focus } = live.current;
    for (const [key, R] of Object.entries(world.regions)) {
      const s = status[key as Region] ?? "unknown";
      // Calm by default: nominal stays near-silver, only what needs attention takes full colour.
      const target = s === "nominal" ? (R.look.shell ? palette.calmShell : palette.calmOrgan) : palette.status[s];
      const hot = s === "watch" || s === "caution" || s === "urgent";
      R.color.lerp(target, k);
      // Focus: everything else steps back so the selected organ reads.
      const back = focused.current && key !== focus?.region ? (R.look.shell ? 0.45 : 0.28) : 1;
      R.fade += ((lit ? (lit.has(key as Region) ? 1 : 0.12) : 1) * back - R.fade) * k;
      R.sel += ((key === selected ? 1 : 0) - R.sel) * k;
      const pressed = key === st.current.pressed;
      R.hov += ((pressed ? 2.2 : key === st.current.hovered ? 1 : 0) - R.hov) * (pressed ? 1 : k);
      const u = R.uniforms;
      (u.uC.value as THREE.Color).copy(R.color).multiplyScalar(R.fade);
      u.uSel.value = R.sel;
      u.uHov.value = R.hov;
      u.uT.value = reduce ? 0.35 : t;
      // Pulse about 0.4 Hz (do not slow it: vestibular discomfort). Static glow with reduced motion.
      u.uGlow.value = hot ? (reduce ? 0.18 : 0.12 + 0.12 * (0.5 + 0.5 * Math.sin(t * 2.4))) : 0;
    }
  });

  return (
    <>
      <primitive object={world.root} />
      <OrbitControls ref={controls} enableDamping dampingFactor={0.08} enablePan={false} enableZoom={false}
        rotateSpeed={0.7} autoRotateSpeed={0.35} minPolarAngle={Math.PI * 0.14} maxPolarAngle={Math.PI * 0.86}
        onStart={() => { st.current.dragging = true; st.current.moved = true; }}
        onEnd={() => { st.current.dragging = false; }} />
    </>
  );
}

export default function BodyCanvas(props: BodyCanvasProps) {
  return (
    <Canvas camera={{ fov: 30, near: 0.01, far: 100, position: [0, 0, 5] }} dpr={[1, 2]}
      gl={{ antialias: true, alpha: true }} aria-hidden="true">
      <Suspense fallback={null}><Body {...props} /></Suspense>
    </Canvas>
  );
}
