import { Pressable, StyleSheet, Text, View } from "react-native";
import { colors, radius } from "./theme";

// Shown when the website cannot be loaded. No status is ever computed here:
// the website runs the engine itself.
export function Unreachable({ url, onRetry }: { url: string; onRetry: () => void }) {
  return (
    <View style={styles.root}>
      <Text style={styles.title}>Website unreachable</Text>
      <Text style={styles.url}>{url}</Text>
      <Pressable accessibilityRole="button" style={styles.retry} onPress={onRetry}>
        <Text style={styles.retryText}>Retry</Text>
      </Pressable>
    </View>
  );
}

const styles = StyleSheet.create({
  root: { position: "absolute", top: 0, left: 0, right: 0, bottom: 0, backgroundColor: colors.bg, justifyContent: "center", padding: 24 },
  title: { color: colors.text, fontSize: 20, fontWeight: "600" },
  url: { color: colors.textMuted, marginTop: 8 },
  retry: {
    marginTop: 24,
    minHeight: 48,
    justifyContent: "center",
    alignItems: "center",
    borderColor: colors.accent,
    borderWidth: 1,
    borderRadius: radius,
  },
  retryText: { color: colors.accent, fontSize: 16, fontWeight: "600" },
});
