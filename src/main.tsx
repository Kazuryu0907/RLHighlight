import React, { Suspense } from "react";
import ReactDOM from "react-dom/client";
import App from "./App";

import {getVersion} from "@tauri-apps/api/app";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <Suspense>
      <App versionPromise={getVersion()} />
    </Suspense>
  </React.StrictMode>,
);
