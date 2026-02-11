import { Button } from "./components/ui/button";
import { useCallback, useState, type SubmitEventHandler } from "react";
import { generate } from "./lib/generate";
import MacAddressInput from "./components/mac-address-input";
import {
  Field,
  FieldDescription,
  FieldError,
  FieldGroup,
  FieldLabel,
  FieldLegend,
  FieldSet,
} from "./components/ui/field";
import { Checkbox } from "./components/ui/checkbox";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "./components/ui/select";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "./components/ui/popover";
import { ArrowUpRight, ChevronDownIcon } from "lucide-react";
import { Calendar } from "./components/ui/calendar";
import { format } from "date-fns";
import { toast } from "sonner";
import { useLocalStorage } from "@uidotdev/usehooks";
import { get_supported_versions } from "../../wasm/pkg/wasm";

const SUPPORTED_VERSIONS = get_supported_versions();

function App() {
  const [success, setSuccess] = useState(false);

  const [sysVersion, setSysVersion] = useLocalStorage(
    "wilbrand/sysVersion",
    "",
  );
  const [sysRegion, setSysRegion] = useLocalStorage("wilbrand/sysRegion", "");
  const [mac, setMac] = useLocalStorage("wilbrand/mac", [
    "",
    "",
    "",
    "",
    "",
    "",
  ]);
  const [date, setDate] = useState<Date>(new Date());

  const [log, setLog] = useState<string[]>([]);
  const logCallback = useCallback((msg: string) => {
    setLog((prev) => [...prev, msg]);
  }, []);

  const version = `${sysVersion}${sysRegion}`;
  const unsupportedVersion =
    sysVersion.length > 0 &&
    sysRegion.length > 0 &&
    !SUPPORTED_VERSIONS.includes(version);

  const handleFormSubmit = useCallback<SubmitEventHandler<HTMLFormElement>>(
    async (e) => {
      e.preventDefault();
      // const formData = new FormData(e.target);
      // const bundleHackMii = formData.get("bundleHackMii") === "on";
      const macString = mac.join("-");
      setLog([]);
      const dateString = format(date, "dd-MM-yyyy");
      const ok = await generate(
        macString,
        dateString,
        version,
        false, // bundleHackMii,
        logCallback,
      );
      if (ok) {
        toast.success("Successfully generated payload!");
        setSuccess(true);
      }
    },
    [logCallback, date, version, mac],
  );

  const disabled =
    mac.some((field) => field.length < 2) ||
    unsupportedVersion ||
    !sysVersion.length ||
    !sysRegion.length;

  return (
    <div>
      <div className="w-full max-w-md mx-auto pt-16 pb-16 px-2">
        <div className="min-h-[calc(100svh-200px)]">
          <div className="mb-8 flex flex-col items-center gap-1">
            <img src="/wilbrand-icon.png" className="h-16 -rotate-2" />
            <h1 className="tracking-tight text-2xl font-semibold">
              Wilbrand Wasm
            </h1>
          </div>
          <form onSubmit={handleFormSubmit}>
            <FieldGroup>
              <FieldSet>
                <FieldLegend>System Information</FieldLegend>
                <FieldDescription>
                  Required information to construct, sign, and encrypt the
                  payload.
                </FieldDescription>
                <FieldGroup>
                  <Field>
                    <FieldLabel>System Menu Version</FieldLabel>
                    <div className="flex items-center gap-1">
                      <Select
                        value={sysVersion}
                        onValueChange={setSysVersion}
                        name="sysVersion"
                      >
                        <SelectTrigger aria-invalid={unsupportedVersion}>
                          <SelectValue placeholder="X.X" />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectGroup>
                            <SelectItem value="4.3">4.3</SelectItem>
                            <SelectItem value="4.2">4.2</SelectItem>
                            <SelectItem value="4.1">4.1</SelectItem>
                            <SelectItem value="4.0">4.0</SelectItem>
                            <SelectItem value="3.5">3.5</SelectItem>
                            <SelectItem value="3.4">3.4</SelectItem>
                            <SelectItem value="3.3">3.3</SelectItem>
                            <SelectItem value="3.2">3.2</SelectItem>
                            <SelectItem value="3.1">3.1</SelectItem>
                            <SelectItem value="3.0">3.0</SelectItem>
                          </SelectGroup>
                        </SelectContent>
                      </Select>
                      <Select
                        value={sysRegion}
                        onValueChange={setSysRegion}
                        name="sysRegion"
                      >
                        <SelectTrigger aria-invalid={unsupportedVersion}>
                          <SelectValue placeholder="X" />
                        </SelectTrigger>
                        <SelectContent>
                          <SelectGroup>
                            <SelectItem value="u">U</SelectItem>
                            <SelectItem value="e">E</SelectItem>
                            <SelectItem value="j">J</SelectItem>
                            <SelectItem value="k">K</SelectItem>
                          </SelectGroup>
                        </SelectContent>
                      </Select>
                    </div>
                    {unsupportedVersion && (
                      <FieldError>
                        Version {version.toUpperCase()} is not supported
                      </FieldError>
                    )}
                  </Field>
                </FieldGroup>
                <FieldGroup>
                  <Field>
                    <FieldLabel>MAC Address</FieldLabel>
                    <MacAddressInput mac={mac} setMac={setMac} />
                  </Field>
                </FieldGroup>
                <FieldGroup>
                  <Field>
                    <FieldLabel>System Date</FieldLabel>
                    <div>
                      <Popover>
                        <PopoverTrigger asChild>
                          <Button
                            variant="outline"
                            data-empty={!date}
                            className="data-[empty=true]:text-muted-foreground w-32 justify-between text-left font-normal"
                          >
                            {date ? (
                              format(date, "dd/MM/yyyy")
                            ) : (
                              <span>Pick a date</span>
                            )}
                            <ChevronDownIcon />
                          </Button>
                        </PopoverTrigger>
                        <PopoverContent className="w-auto p-0" align="start">
                          <Calendar
                            required
                            mode="single"
                            selected={date}
                            onSelect={setDate}
                            defaultMonth={date}
                          />
                        </PopoverContent>
                      </Popover>
                    </div>
                  </Field>
                </FieldGroup>
                {/* <FieldSet>
                <FieldLegend>Bundle Options</FieldLegend>
                <FieldGroup> */}
                <Field orientation="horizontal">
                  <Checkbox
                    id="bundle-hackmii-checkbox"
                    disabled
                    name="bundleHackMii"
                  />
                  <FieldLabel
                    htmlFor="bundle-hackmii-checkbo"
                    className="font-normal"
                  >
                    Bundle HackMii Installer v1.2
                  </FieldLabel>
                </Field>
                {/* </FieldGroup>
              </FieldSet> */}
              </FieldSet>
              <Field orientation="horizontal">
                <Button
                  type="submit"
                  variant="outline"
                  className="flex-1"
                  size="lg"
                  disabled={disabled}
                >
                  Cut the White Wire
                </Button>
                <Button
                  type="submit"
                  className="flex-1"
                  size="lg"
                  disabled={disabled}
                >
                  Cut the Black Wire
                </Button>
              </Field>
            </FieldGroup>
          </form>
          {success && (
            <div className="mt-8">
              <p className="font-medium mb-2">Instructions</p>
              <ol className="list-decimal pl-4 text-sm">
                <li>
                  Extract the zip file and place the{" "}
                  <span className="font-mono font-semibold text-xs bg-accent border rounded px-1 py-0.5">
                    private
                  </span>{" "}
                  folder into the root of your SD card
                </li>
                <li>
                  Copy{" "}
                  <span className="font-mono font-semibold text-xs bg-accent border rounded px-1 py-0.5">
                    boot.elf
                  </span>{" "}
                  from the{" "}
                  <a
                    href="https://bootmii.org/download/"
                    target="_blank"
                    className="underline underline-offset-2"
                  >
                    HackMii Installer v1.2{" "}
                    <ArrowUpRight className="size-3 inline-block -top-1 relative" />
                  </a>{" "}
                  and place it into the root of your SD card
                </li>
              </ol>
            </div>
          )}
          {log.length > 0 && (
            <div className="mt-8">
              <p className="font-medium mb-2">Log Output</p>
              <pre className="text-xs break-all whitespace-pre-wrap p-4 rounded text-white bg-accent-foreground">
                {log.join("\n")}
              </pre>
            </div>
          )}
        </div>
      </div>

      <div className="bg-secondary border-t">
        <div className="py-16 sm:py-24 p-4 max-w-4xl m-auto text-muted-foreground text-xs">
          <div className="grid sm:grid-cols-3 gap-4 grid-cols-1">
            <div>
              <p>
                Built by liangchunn in{" "}
                <span className="font-medium">Rust + WebAssembly</span>
              </p>
              <br />
              <p>
                <a
                  href="https://github.com/liangchunn"
                  target="_blank"
                  className="hover:underline underline-offset-2"
                >
                  View in GitHub
                </a>
                <ArrowUpRight className="size-3 inline-block -top-1 relative" />
              </p>
            </div>
            <div>
              <p>
                <a
                  href="https://github.com/giantpune/mailboxbomb"
                  target="_blank"
                  className="hover:underline underline-offset-2"
                >
                  Originally built in C++ by giantprune
                  <ArrowUpRight className="size-3 inline-block -top-1 relative" />
                </a>
              </p>
              <p>
                <a
                  href="https://wilbrand.donut.eu.org"
                  target="_blank"
                  className="hover:underline underline-offset-2"
                >
                  Original web version built by emilydaemon
                  <ArrowUpRight className="size-3 inline-block -top-1 relative" />
                </a>
              </p>
              <p>
                <a
                  href="https://github.com/giantpune/mailboxbomb/issues/1#issuecomment-3104792286"
                  target="_blank"
                  className="hover:underline underline-offset-2"
                >
                  Envelope image by leahanderson1 in this issue
                  <ArrowUpRight className="size-3 inline-block -top-1 relative" />
                </a>
              </p>
            </div>
            <div>
              <p>
                <span className="underline font-medium underline-offset-2">
                  Disclaimer
                </span>
                : I am not responsible for your console bricking, or any form of
                software/hardware damage that may be caused by the software
                provided. Use at your own risk!
              </p>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
