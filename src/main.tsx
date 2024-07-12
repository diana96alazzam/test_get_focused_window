import ReactDOM from "react-dom/client";
import { invoke } from "@tauri-apps/api/tauri";

const root = ReactDOM.createRoot(document.getElementById("root")!);

root.render(
  <div>
    <button
      onClick={async () => {
        await invoke("test_cmd", {
          automationComponent: "",
        }).then(res => {
          console.log("Test command response:", res);
        }).catch(err => {
          console.log("Test command error:", err);
        });
      }}
    >
      Click here
    </button>
  </div>
);
