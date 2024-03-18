import { useState } from "react";
import { Choice } from "./store";

export const ChoiceView = ({
  choice,
  active,
  selected,
}: {
  choice: Choice;
  active: boolean;
  selected: boolean;
}) => {
  const disabled = !selected && choice.chosen;

  return (
    <button
      style={{
        backgroundColor: active
          ? choice.chosen
            ? "#4CAF50"
            : "#8BC34A"
          : choice.chosen
            ? "#F44336"
            : "#E57373",
        color: "white",
        opacity: disabled ? 0.5 : 1,
        cursor: disabled ? "not-allowed" : "pointer",
      }}
      disabled={disabled}
    >
      {choice.label}
    </button>
  );
};

export const ChoicesView = ({ choices, active }: { choices: Choice[]; active: boolean }) => {
  const [selected, setSelected] = useState<number>(0);

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === "ArrowUp") {
      setSelected((prev) => (prev - 1 + choices.length) % choices.length);
    } else if (e.key === "ArrowDown") {
      setSelected((prev) => (prev + 1) % choices.length);
    }
  };

  if (active) {
    window.addEventListener("keydown", handleKeyDown);
  } else {
    window.removeEventListener("keydown", handleKeyDown);
  }

  return (
    <div>
      {choices.map((choice, ix) => (
        <ChoiceView key={choice.id} choice={choice} active={active} selected={selected === ix} />
      ))}
    </div>
  );
};
