import { StatusBar } from "expo-status-bar";
import { useEffect, useState } from "react";
import { StyleSheet, Text } from "react-native";
import { SafeAreaProvider, SafeAreaView } from "react-native-safe-area-context";
import { sis } from "./src/api";
import { statusLabel } from "./src/status";

// App shell: layout and navigation only. Put each domain's screens in
// src/features/<domain>/ so teams can work without editing the same files.
export default function App() {
  const [apiStatus, setApiStatus] = useState("checking…");

  useEffect(() => {
    sis
      .health()
      .then((h) => setApiStatus(statusLabel(h)))
      .catch(() => setApiStatus("unreachable, is the backend running?"));
  }, []);

  return (
    <SafeAreaProvider>
      <SafeAreaView style={styles.container}>
        <Text style={styles.title}>Student Information System</Text>
        <Text>API: {apiStatus}</Text>
        <StatusBar style="auto" />
      </SafeAreaView>
    </SafeAreaProvider>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: "#fff", paddingHorizontal: 16 },
  title: { fontSize: 22, fontWeight: "600", marginBottom: 16 },
});
