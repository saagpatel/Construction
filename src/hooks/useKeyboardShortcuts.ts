import { useEffect } from "react";
import type { NavigateFunction } from "react-router-dom";

interface KeyboardShortcut {
  key: string;
  metaKey?: boolean;
  ctrlKey?: boolean;
  shiftKey?: boolean;
  action: () => void;
  description: string;
}

export function useKeyboardShortcuts(shortcuts: KeyboardShortcut[]) {
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      for (const shortcut of shortcuts) {
        const metaMatch =
          shortcut.metaKey === undefined || shortcut.metaKey === event.metaKey;
        const ctrlMatch =
          shortcut.ctrlKey === undefined || shortcut.ctrlKey === event.ctrlKey;
        const shiftMatch =
          shortcut.shiftKey === undefined ||
          shortcut.shiftKey === event.shiftKey;
        const keyMatch = event.key.toLowerCase() === shortcut.key.toLowerCase();

        if (metaMatch && ctrlMatch && shiftMatch && keyMatch) {
          event.preventDefault();
          shortcut.action();
          break;
        }
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [shortcuts]);
}

// Global shortcuts factory - requires navigate function from useNavigate()
export function createGlobalShortcuts(
  navigate: NavigateFunction,
): KeyboardShortcut[] {
  return [
    {
      key: "n",
      metaKey: true,
      action: () => navigate("/incidents/new"),
      description: "New Incident",
    },
    {
      key: "k",
      metaKey: true,
      action: () => {
        const searchInput = document.querySelector(
          'input[type="search"], input[placeholder*="Search"]',
        ) as HTMLInputElement | null;
        searchInput?.focus();
      },
      description: "Focus Search",
    },
    {
      key: "d",
      metaKey: true,
      action: () => navigate("/dashboard"),
      description: "Go to Dashboard",
    },
    {
      key: "i",
      metaKey: true,
      action: () => navigate("/incidents"),
      description: "Go to Incidents",
    },
    {
      key: "o",
      metaKey: true,
      action: () => navigate("/osha"),
      description: "Go to OSHA Forms",
    },
  ];
}
