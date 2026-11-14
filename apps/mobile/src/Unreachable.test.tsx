import { act, create } from "react-test-renderer";
import { Unreachable } from "./Unreachable";

test("shows the URL and a Retry button that retries", async () => {
  const onRetry = jest.fn();
  let r!: ReturnType<typeof create>;
  await act(async () => {
    r = create(<Unreachable url="http://x.test" onRetry={onRetry} />);
  });
  expect(JSON.stringify(r.toJSON())).toContain("http://x.test");
  await act(async () => r.root.findByProps({ accessibilityRole: "button" }).props.onPress());
  expect(onRetry).toHaveBeenCalled();
});
