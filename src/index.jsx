import { createRoot } from "react-dom/client";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import "./index.scss";

const App = () => {
  const [activation, setActivation] = useState("C");
  const [closeToTray, setCloseToTray] = useState(true);

  const handleSelect = (event) => {
    const val = event.target.value;
    setActivation(val);
    invoke("set_var", { key: "activation", val });
  };

  const handleChecked = () => {
    setCloseToTray(!closeToTray);
    invoke("set_var", { key: "closetotray", val: String(!closeToTray) });
  };

  return (
    <>
      <h1>Настройки</h1>
      <hr />
      <div>
        <p>Активация:</p>
        <select value={activation} onChange={handleSelect}>
          <option value="C">WIN+ALT+C</option>
          <option value="S">WIN+ALT+S</option>
          <option value="L">WIN+ALT+L</option>
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
