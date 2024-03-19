import { useEffect, useState } from "react";
import { Choice } from "./store";
import { theme } from "./theme";
import clsx from "clsx";

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

  let text_color = "text-zinc-400";

  if (choice.chosen || selected) {
    text_color = "text-zinc-50";
  }

  return (
    <div className="group flex gap-1">
      <div className="-mr-2 h-4 w-4 flex-none text-orange-500">{current && selected && ">"}</div>
      <button
        className={clsx(text_color)}
        style={{
          opacity: disabled ? 0.5 : 1,
          cursor: disabled ? "not-allowed" : "pointer",
        }}
        disabled={disabled}
      >
        {choice.label}
      </button>
    </div>
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
  let debounceTimer: number | undefined;

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
