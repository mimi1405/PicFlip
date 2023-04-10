import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import { Routes, Route } from "react-router";
import "./App.css";
import Main from "./screens/Main";

function App() {

  return (
    <>
    <main>
      <Routes>
        <Route path="/" element={<Main/>} />
      </Routes>
    </main>
    </>
  );
}

export default App;