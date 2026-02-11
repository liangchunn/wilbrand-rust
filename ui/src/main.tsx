import "@fontsource-variable/geist";
import "@fontsource-variable/jetbrains-mono";
import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import "./index.css";
import App from "./App.tsx";
import { Toaster } from "./components/ui/sonner.tsx";
import { ScrollArea } from "./components/ui/scroll-area.tsx";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <div className="h-screen">
      <ScrollArea className="h-full">
        <App />
        <Toaster richColors position="top-center" theme="light" />
      </ScrollArea>
    </div>
  </StrictMode>,
);
