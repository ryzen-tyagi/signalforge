export default function LoginPage() {
  return (
    <main className="main">
      <h1 className="page-title">SignalForge</h1>
      <form className="form card" style={{ marginTop: 24 }}>
        <input className="input" type="email" placeholder="Email" defaultValue="demo@signalforge.local" />
        <input className="input" type="password" placeholder="Password" defaultValue="signalforge" />
        <button className="button" type="button">Login</button>
      </form>
    </main>
  );
}

