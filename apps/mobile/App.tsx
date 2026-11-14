import { StatusBar } from "expo-status-bar";
import {
  type ComponentType,
  type Ref,
  useEffect,
  useRef,
  useState,
} from "react";
import {
  BackHandler,
  Linking,
  StatusBar as RNStatusBar,
  StyleSheet,
  View,
} from "react-native";
import { WebView as AnyWebView } from "react-native-webview";
import type { AndroidWebViewProps } from "react-native-webview/lib/WebViewTypes";
import { Unreachable } from "./src/Unreachable";
import { colors } from "./src/theme";

// ponytail: the package types the cross-platform export as `never` under TS 6; Android only here.
type Web = { goBack(): void; reload(): void };
const WebView = AnyWebView as unknown as ComponentType<
  AndroidWebViewProps & { ref?: Ref<Web> }
>;

// Dev builds reach the host's Vite server from the emulator. Release builds must set
// EXPO_PUBLIC_WEB_URL=https://ahc-37u.pages.dev at build time.
const WEB_URL = process.env.EXPO_PUBLIC_WEB_URL ?? "http://10.0.2.2:5180";
const ORIGIN = new URL(WEB_URL).origin;

export default function App() {
  const web = useRef<Web>(null);
  const canGoBack = useRef(false);
  const [failed, setFailed] = useState(false);

  useEffect(() => {
    const sub = BackHandler.addEventListener("hardwareBackPress", () => {
      if (!canGoBack.current) return false; // default: leave the app
      web.current?.goBack();
      return true;
    });
    return () => sub.remove();
  }, []);

  return (
    <View style={styles.root}>
      <StatusBar style="light" />
      <WebView
        ref={web}
        source={{ uri: WEB_URL }}
        style={styles.black}
        containerStyle={styles.black}
        javaScriptEnabled
        domStorageEnabled
        onNavigationStateChange={(s) => {
          canGoBack.current = s.canGoBack;
        }}
        onShouldStartLoadWithRequest={(r) => {
          if (r.url.startsWith(ORIGIN) || !/^https?:/.test(r.url)) return true;
          void Linking.openURL(r.url);
          return false;
        }}
        onError={() => setFailed(true)}
        onHttpError={() => setFailed(true)}
      />
      {failed ? (
        <Unreachable
          url={WEB_URL}
          onRetry={() => {
            setFailed(false);
            web.current?.reload();
          }}
        />
      ) : null}
    </View>
  );
}

const styles = StyleSheet.create({
  black: { flex: 1, backgroundColor: colors.bg },
  root: {
    flex: 1,
    backgroundColor: colors.bg,
    paddingTop: RNStatusBar.currentHeight ?? 0,
  },
});
