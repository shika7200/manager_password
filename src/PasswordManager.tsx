import React, { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

type Password = {
  id: number;
  title: string;
  login: string;
  password: string;
};

const PasswordManager: React.FC = () => {
  const [passwords, setPasswords] = useState<Password[]>([]);
  const [title, setTitle] = useState<string>("");
  const [login, setLogin] = useState<string>("");
  const [password, setPassword] = useState<string>("");
  const [editId, setEditId] = useState<number | null>(null);

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
      if (editId === null) {
        await invoke("add_credential_command", { title, login, password });
      } else {
        await invoke("update_credential_command", { id: editId, title, login, password });
      }
      fetchPasswords();
      setTitle("");
      setLogin("");
      setPassword("");
      setEditId(null);
    } catch (error) {
      console.error("Failed to save password:", error);
    }
  };

  const startEdit = (id: number, title: string, login: string, password: string) => {
    setEditId(id);
    setTitle(title);
    setLogin(login);
    setPassword(password);
  };

  const deletePassword = async (id: number) => {
    try {
      await invoke("delete_credential_command", { id });
      fetchPasswords();
    } catch (error) {
      console.error("Failed to delete password:", error);
    }
  };

  return (
    <div className="app-container">
      <h1>Менеджер паролей</h1>
      <div className="password-manager-container">
        <input
          type="text"
          placeholder="Название"
          value={title}
          onChange={(e) => setTitle(e.target.value)}
        />
        <input
          type="text"
          placeholder="Логин"
          value={login}
          onChange={(e) => setLogin(e.target.value)}
        />
        <input
          type="password"
          placeholder="Пароль"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        />
        <button onClick={savePassword}>{editId === null ? "Сохранить" : "Обновить"}</button>
      </div>
      <div>
        <h2>Сохраненные пароли</h2>
        <ul>
          {passwords.map((pass) => (
            <li key={pass.id}>
              <strong>{pass.title}</strong>: {pass.login} - {pass.password}
              <button onClick={() => startEdit(pass.id, pass.title, pass.login, pass.password)}>Изменить</button>
              <button onClick={() => deletePassword(pass.id)}>Удалить</button>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
};

export default PasswordManager;
