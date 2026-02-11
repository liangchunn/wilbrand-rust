import { Fragment, useRef, type Dispatch, type SetStateAction } from "react";
import { Input } from "@/components/ui/input";
import { MinusIcon } from "lucide-react";

const placeholder = ["AA", "BB", "CC", "DD", "EE", "FF"];

type MacAddressInputProps = {
  mac: string[];
  setMac: Dispatch<SetStateAction<string[]>>;
};

export default function MacAddressInput({ mac, setMac }: MacAddressInputProps) {
  const inputsRef = useRef<(HTMLInputElement | null)[]>([]);

  const handleChange = (index: number, value: string) => {
    const hex = value
      .replace(/[^0-9a-fA-F]/g, "")
      .slice(0, 2)
      .toUpperCase();
    setMac((currMac) => {
      const newMac = [...currMac];
      newMac[index] = hex;
      return newMac;
    });

    // advance if 2 chars entered
    if (hex.length === 2 && index < 5) {
      inputsRef.current[index + 1]?.focus();
    }
  };

  const handleKeyDown = (index: number, e: React.KeyboardEvent) => {
    if (e.key === "Backspace" && !mac[index] && index > 0) {
      inputsRef.current[index - 1]?.focus();
    }
  };

  return (
    <div className="flex items-center gap-1">
      {mac.map((value, i) => (
        <Fragment key={i}>
          <Input
            value={value}
            onChange={(e) => handleChange(i, e.target.value)}
            onKeyDown={(e) => handleKeyDown(i, e)}
            className="w-10 text-center font-mono text-md! px-1 py-1"
            maxLength={2}
            ref={(el) => {
              inputsRef.current[i] = el;
            }}
            name={`mac-${i}`}
            placeholder={placeholder[i]}
          />
          {i < 5 && <MinusIcon className="text-secondary-foreground size-3" />}
        </Fragment>
      ))}
    </div>
  );
}
