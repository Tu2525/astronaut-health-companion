import type { Assessment } from "@contract/Assessment";
import { StatusBar } from "expo-status-bar";
import { useCallback, useEffect, useState } from "react";
import { Pressable, SafeAreaView, StyleSheet, Text } from "react-native";
import { assess, getDemo } from "./src/api";
import { Home } from "./src/Home";
import { colors, radius, space, touchTarget } from "./src/theme";

export default function App() {
  const [assessment, setAssessment] = useState<Assessment | null>(null);
  const [failed, setFailed] = useState(false);

  const load = useCallback(async () => {
    try {
      const d = await getDemo();
      setAssessment(
        await assess({
          crew_id: d.crew_id,
          events: d.events,
          now_met_s: d.now_met_s,
          kit: d.kit,
          registry: null,
        }),
      );
    } catch {
      // Never guess a status locally: show that the engine is out of reach.
      setAssessment(null);
      setFailed(true);
    }
  }, []);

  useEffect(() => {
    // Fetches once on mount; state is set only after the awaited request.
    // eslint-disable-next-line react-hooks/set-state-in-effect
    void load();
  }, [load]);

  return (
    <SafeAreaView style={styles.root}>
      <StatusBar style="light" />
      <Text style={styles.brand}>ZeroG Pulse</Text>
      {assessment ? <Home assessment={assessment} /> : null}
      {failed ? (
        <>
          <Text style={styles.error}>Engine unreachable</Text>
          <Pressable style={styles.retry} onPress={() => {
              setFailed(false);
              void load();
            }}>
            <Text style={styles.retryText}>Retry</Text>
          </Pressable>
        </>
      ) : null}
      {!assessment && !failed ? <Text style={styles.muted}>Loading…</Text> : null}
    </SafeAreaView>
  );
}

const styles = StyleSheet.create({
  root: { flex: 1, backgroundColor: colors.bg, padding: space[6], paddingTop: space[8] * 2 },
  brand: { color: colors.text, fontSize: 24, fontWeight: "700", marginBottom: space[6] },
  muted: { color: colors.textMuted },
  error: { color: colors.text, fontSize: 18, fontWeight: "600" },
  retry: {
    marginTop: space[4],
    minHeight: touchTarget,
    justifyContent: "center",
    alignItems: "center",
    backgroundColor: colors.surfaceRaised,
    borderColor: colors.accent,
    borderWidth: 1,
    borderRadius: radius.md,
  },
  retryText: { color: colors.accent, fontSize: 16, fontWeight: "600" },
});
