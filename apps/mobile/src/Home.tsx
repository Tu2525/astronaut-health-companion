import type { Assessment } from "@contract/Assessment";
import { StyleSheet, Text, View } from "react-native";
import { StatusChip } from "./StatusChip";
import { colors, radius, space } from "./theme";

export function Home({ assessment }: { assessment: Assessment }) {
  const top = assessment.flags[0];
  return (
    <View>
      <StatusChip status={assessment.overall} />
      <Text style={styles.muted}>{`Escalation: ${assessment.escalation}`}</Text>
      {top ? (
        <View style={styles.card}>
          <Text style={styles.title}>{top.summary}</Text>
          <Text style={styles.body}>{`Next step: ${top.next_step}`}</Text>
        </View>
      ) : null}
    </View>
  );
}

const styles = StyleSheet.create({
  muted: { color: colors.textMuted, marginTop: space[3] },
  card: {
    marginTop: space[4],
    padding: space[4],
    backgroundColor: colors.surface,
    borderColor: colors.border,
    borderWidth: 1,
    borderRadius: radius.md,
  },
  title: { color: colors.text, fontSize: 16, fontWeight: "600" },
  body: { color: colors.textMuted, marginTop: space[2] },
});
