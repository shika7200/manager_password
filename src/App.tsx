import "./App.css";
import PasswordManager from "./PasswordManager";
function App() {
  return (
    <div className="app-container">
      <h1>
        Менеджер паролей
      </h1>
      <div className="password-manager-container">
        <PasswordManager />
      </div>
    </div>
  );
}

export default App;
