import { useEffect, useState, useCallback } from "react";
import { Choice, useGameState } from "./store";
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
  const disabled = !current;

  const text_color = choice.chosen || selected ? "text-zinc-50" : "text-zinc-400";

  return (
    <div className="group flex gap-1">
      <div className="-mr-2 h-4 w-4 flex-none text-orange-500">{current && selected && ">"}</div>
      <button
        className={clsx(text_color, {
          "cursor-not-allowed opacity-50": disabled,
          "cursor-pointer": !disabled,
        })}
        disabled={disabled}
      >
        {choice.label}
      </button>
    </div>
  );
};

export const ChoicesView = ({ choices }: { choices: Choice[] }) => {
  const [selected, setSelected] = useState(0);
  const { setChosenChoice, setCurrentChoices, currentChoices } = useGameState();

  const current = currentChoices[0].group_id === choices[0].group_id;

  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      console.log("fired handleKeyDown");

      if (e.key === "ArrowUp") {
        setSelected((prev) => (prev - 1 + choices.length) % choices.length);
      } else if (e.key === "ArrowDown") {
        setSelected((prev) => (prev + 1) % choices.length);
      } else if (e.key === "Enter") {
        setChosenChoice(choices[selected]?.id);
        setCurrentChoices("");
      }
    },
    [choices, selected, setChosenChoice],
  );

  useEffect(() => {
    if (current) {
      window.addEventListener("keydown", handleKeyDown);
      return () => window.removeEventListener("keydown", handleKeyDown);
    }
  }, [current, handleKeyDown]);

  return (
    <div className="flex flex-col justify-start gap-1 py-4 text-left">
      {choices.map((choice, ix) => (
        <ChoiceView key={choice.id} choice={choice} current={current} selected={selected === ix} />
      ))}
    </div>
  );
};
