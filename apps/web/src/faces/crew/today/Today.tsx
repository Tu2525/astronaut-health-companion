import { useMemo } from "react";
import { demo, dueTasks } from "../../../engine";
import { formatMet } from "../../../clock";
import type { CrewStore } from "../../../store";
import { Card } from "../../../ui/Card";
import "./today.css";

export function Today({ store }: { store: CrewStore }) {
  const { settings, events, nowMet } = store.getSnapshot();
  const kit = useMemo(() => demo().kit, []);
  const tasks = dueTasks({ crew_id: settings.crew_id, events, now_met_s: nowMet, kit, registry: null });
  const routine = tasks.filter((task) => task.kind === "routine");
  const bonus = tasks.filter((task) => task.kind === "bonus");
  const href = (id: string | null) => (id ? `#/crew/measure/${id}` : "#/crew/checkin");

  return (
    <>
      <div className="page-heading">
        <h1>Today</h1>
        <span className="data-label">Simulated mission data</span>
      </div>
      <div className="today-grid">
        <Card>
          <h2>Routine</h2>
          <ol className="task-list">
            {routine.map((task) => (
              <li key={task.id} className={task.completed ? "task done" : "task"}>
                <a href={href(task.indicator_id)}>
                  <span className="task-time">{formatMet(task.due_met_s)}</span>
                  <span className="task-label">{task.label}</span>
                  <span className="task-state">{task.completed ? "✓ Done" : "To do"}</span>
                </a>
              </li>
            ))}
          </ol>
        </Card>
        {bonus.length > 0 && (
          <Card>
            <h2>Bonus</h2>
            <p className="next-step">Bonus — never breaks your streak.</p>
            <ul className="task-list">
              {bonus.map((task) => (
                <li key={task.id} className="task bonus">
                  <a href={href(task.indicator_id)}>
                    <span className="task-time">Bonus</span>
                    <span className="task-label">{task.label}</span>
                    {task.reason && <span className="task-reason">{task.reason}</span>}
                  </a>
                </li>
              ))}
            </ul>
          </Card>
        )}
      </div>
    </>
  );
}
