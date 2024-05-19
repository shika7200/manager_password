import React, { useEffect, useState } from "react";
import { Password } from "./types";
import { invoke } from "@tauri-apps/api/tauri";

const PasswordManager: React.FC = () => {
  const [passwords, setPasswords] = useState<Password[]>([]);
  const [title, setTitle] = useState<string>("");
  const [password, setPassword] = useState<string>("");

  useEffect(() => {
    fetchPasswords();
  }, []);

  const fetchPasswords = async () => {
    setPasswords(await invoke("getPasswords", {}));
  };

  const savePassword = async () => {
    await invoke("save_password", { password, title });
    setPasswords(await invoke("getPasswords", {}));
  };

  return (
    <div>
      <h1>Менеджер паролей</h1>
      <div>
        <input
          type="text"
          placeholder="Название"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
        />
        <input
          type="password"
          placeholder="Пароль"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
        <button onClick={savePassword}>Сохранить</button>
      </div>

      <div>
        <h2>Сохраненные пароли</h2>
        <ul>
          {passwords.map((pass) => (
            <li key={pass.id}>
              <strong>{pass.title}</strong>: {pass.password}
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};

export default PasswordManager;
