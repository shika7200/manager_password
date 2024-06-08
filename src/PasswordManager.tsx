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
    setPasswords(await invoke("get_credential_command", {}));
  };

  const savePassword = async () => {
    await invoke("add_credential_command", {title: title, password: password  });
    setPasswords(await invoke("get_credential_command", {}));
    setTitle("");
    setPassword("");
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
