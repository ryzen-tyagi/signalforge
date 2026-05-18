import { Activity } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";

export default function LoginPage() {
  return (
    <main className="login-screen">
      <Card className="login-panel">
        <CardHeader>
          <CardTitle>
            <Activity size={18} aria-hidden /> SignalForge
          </CardTitle>
          <CardDescription>Sign in to the incident command dashboard.</CardDescription>
        </CardHeader>
        <CardContent>
          <form className="form">
            <Input type="email" placeholder="Email" defaultValue="demo@signalforge.local" />
            <Input type="password" placeholder="Password" defaultValue="signalforge" />
            <Button type="button">Login</Button>
          </form>
        </CardContent>
      </Card>
    </main>
  );
}
