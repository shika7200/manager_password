import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

type Password = {
  id: number;
  title: string;
  password: string;
};

const PasswordManager: React.FC = () => {
  const [passwords, setPasswords] = useState<Password[]>([]);
  const [title, setTitle] = useState<string>("");
  const [password, setPassword] = useState<string>("");

  useEffect(() => {
    fetchPasswords();
  }, []);

  const fetchPasswords = async () => {
    try {
      const fetchedPasswords: Password[] = await invoke("get_credential_command");
      console.log("Fetched passwords:", fetchedPasswords); // Отладочная информация
      setPasswords(fetchedPasswords);
    } catch (error) {
      console.error("Failed to fetch passwords:", error);
    }
  };

  const savePassword = async () => {
    try {
      await invoke("add_credential_command", { title, password });
      console.log("Password saved:", { title, password }); // Отладочная информация
      fetchPasswords();
      setTitle("");
      setPassword("");
    } catch (error) {
      console.error("Failed to save password:", error);
    }
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
