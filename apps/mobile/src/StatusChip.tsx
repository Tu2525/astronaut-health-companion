import type { Status } from "@contract/Status";
import { StyleSheet, Text, View } from "react-native";
import { colors, radius, space } from "./theme";

// Presentation only: colour + icon + word. The status itself comes from the engine.
const LOOK: Record<Status, { icon: string; word: string; color: string }> = {
  nominal: { icon: "●", word: "Nominal", color: colors.statusNominal },
  watch: { icon: "◐", word: "Watch", color: colors.statusWatch },
  caution: { icon: "▲", word: "Caution", color: colors.statusCaution },
  urgent: { icon: "◆", word: "Urgent", color: colors.statusUrgent },
  unknown: { icon: "○", word: "Unknown", color: colors.statusUnknown },
};

export function StatusChip({ status }: { status: Status }) {
  const { icon, word, color } = LOOK[status];
  return (
    <View style={[styles.chip, { backgroundColor: color }]}>
      <Text style={styles.text}>{`${icon} ${word}`}</Text>
    </View>
  );
}

const styles = StyleSheet.create({
  chip: {
    alignSelf: "flex-start",
    paddingHorizontal: space[4],
    paddingVertical: space[2],
    borderRadius: radius.lg,
  },
  text: { color: colors.textOnStatus, fontSize: 20, fontWeight: "700" },
});
