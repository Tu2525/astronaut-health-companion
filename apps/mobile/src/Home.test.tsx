import type { Assessment } from "@contract/Assessment";
import { act, create } from "react-test-renderer";
import fixture from "./fixtures/assess-demo.json";
import { Home } from "./Home";

// The fixture is a saved real /api/assess response on the demo seed.
test("renders the engine overall status as icon + word", async () => {
  let r!: ReturnType<typeof create>;
  await act(async () => {
    r = create(<Home assessment={fixture as Assessment} />);
  });
  const text = JSON.stringify(r.toJSON());
  expect(text).toContain("◐ Watch");
  expect(text).toContain("Next step:");
});
