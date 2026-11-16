import { useEffect, useState } from 'react'
import { sis } from './api'
import { statusLabel } from './status'

// App shell: layout and navigation only. Put each domain's screens in
// src/features/<domain>/ so teams can work without editing the same files.
export default function App() {
  const [apiStatus, setApiStatus] = useState('checking…')

  useEffect(() => {
    sis
      .health()
      .then((h) => setApiStatus(statusLabel(h)))
      .catch(() => setApiStatus('unreachable, is the backend running?'))
  }, [])

  return (
    <main>
      <h1>Student Information System</h1>
      <p className="status">API: {apiStatus}</p>
    </main>
  )
}
