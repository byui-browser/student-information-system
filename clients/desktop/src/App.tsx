import { useEffect, useState } from "react";
import { connect } from "./api";
import { statusLabel } from "./status";
import "./App.css";

// App shell: layout and navigation only. Put each domain's screens in
// src/features/<domain>/ so teams can work without editing the same files.
export default function App() {
  const [apiStatus, setApiStatus] = useState("checking…");

  useEffect(() => {
    connect()
      .then((sis) => sis.health())
      .then((h) => setApiStatus(statusLabel(h)))
      .catch(() => setApiStatus("unreachable, is the backend running?"));
  }, []);

  return (
    <main>
      <h1>Student Information System</h1>
      <p>API: {apiStatus}</p>
    </main>
  );
}
