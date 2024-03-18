export type ChoiceData = {
  id: number;
  label: string;
  content: string;
  requirements: string;
  chosen: boolean;
};

export const Choice = ({ choice }: { choice: ChoiceData }) => {
  // placeholder, will come from game context
  const is_active = true;
  // placeholder, will come from game context
  const requirements_met = true;
  // placeholder, will come from game context
  const is_current = false;

  return (
    <button
      style={{
        backgroundColor: is_active
          ? choice.chosen
            ? "#4CAF50"
            : "#8BC34A"
          : choice.chosen
            ? "#F44336"
            : "#E57373",
        color: "white",
        cursor: is_current && requirements_met ? "pointer" : "not-allowed",
        opacity: requirements_met ? "1" : "0.5",
      }}
      disabled={!is_current || !requirements_met}
    >
      {choice.label}
      {!requirements_met && <p style={{ color: "yellow" }}>Requirements: {choice.requirements}</p>}
    </button>
  );
};
