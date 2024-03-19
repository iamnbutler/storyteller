import { useEffect, useState } from "react";
import { Choice } from "./store";
import { theme } from "./theme";

export const ChoiceView = ({
  choice,
  current,
  selected,
}: {
  choice: Choice;
  current: boolean;
  selected: boolean;
}) => {
  const disabled = !selected && choice.chosen;

  const inactive_chosen = theme.text.primary;
  const inactive_not_chosen = theme.text.muted;
  const active_selected = theme.accent;
  const active_not_selected = theme.text.secondary;

  return (
    <button
      style={{
        background: selected ? theme.accent : "transparent",
        color: current
          ? choice.chosen
            ? active_selected
            : active_not_selected
          : choice.chosen
            ? inactive_chosen
            : inactive_not_chosen,
        opacity: disabled ? 0.5 : 1,
        cursor: disabled ? "not-allowed" : "pointer",
      }}
      disabled={disabled}
      className="text-left"
    >
      {choice.label}
    </button>
  );
};

export const ChoicesView = ({
  choices,
  current: active,
}: {
  choices: Choice[];
  current: boolean;
}) => {
  const [selected, setSelected] = useState<number>(0);

  const debounceTimeout = 50; // Debouncing period in milliseconds
  let debounceTimer: any = null;

  const debounceSetSelected = (value: (prev: number) => number) => {
    clearTimeout(debounceTimer);
    debounceTimer = setTimeout(() => setSelected(value), debounceTimeout);
  };

  // The event handler now calls the debounced version of "setSelected"
  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === "ArrowUp") {
      debounceSetSelected((prev) => (prev - 1 + choices.length) % choices.length);
    } else if (e.key === "ArrowDown") {
      debounceSetSelected((prev) => (prev + 1) % choices.length);
    }
  };

  // Use "useEffect" to manage adding and removing the event listener
  useEffect(() => {
    if (active) {
      window.addEventListener("keydown", handleKeyDown);
      // Clear the debouncing timer on unmount
      return () => {
        window.removeEventListener("keydown", handleKeyDown);
        clearTimeout(debounceTimer);
      };
    }
  }, [active, handleKeyDown]);

  return (
    <div className="flex flex-col justify-start gap-1 py-4 text-left">
      {choices.map((choice, ix) => (
        <ChoiceView key={choice.id} choice={choice} current={active} selected={selected === ix} />
      ))}
    </div>
  );
};
