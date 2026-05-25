import { createRoot } from "react-dom/client";
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { platform } from "@tauri-apps/api/os";
import "./index.scss";

const App = () => {
  const [activation, setActivation] = useState("C");
  const [closeToTray, setCloseToTray] = useState(true);
  const [isMac, setIsMac] = useState(false);

  useEffect(() => {
    platform().then((p) => setIsMac(p === "darwin"));
  }, []);

  const handleSelect = (event) => {
    const val = event.target.value;
    setActivation(val);
    invoke("set_var", { key: "activation", val });
  };

  const handleChecked = () => {
    setCloseToTray(!closeToTray);
    invoke("set_var", { key: "closetotray", val: String(!closeToTray) });
  };

  const mod1 = isMac ? "CMD" : "WIN";
  const mod2 = isMac ? "OPT" : "ALT";

  return (
    <>
      <h1>Настройки</h1>
      <hr />
      <div>
        <p>Активация:</p>
        <select value={activation} onChange={handleSelect}>
          <option value="C">{mod1}+{mod2}+C</option>
          <option value="S">{mod1}+{mod2}+S</option>
          <option value="L">{mod1}+{mod2}+L</option>
        </select>
      </div>
      <div>
        <p>Закрывать в трей:</p>
        <input
          checked={closeToTray}
          onChange={handleChecked}
          type="checkbox"
        ></input>
      </div>
    </>
  );
};

createRoot(document.body).render(<App />);
