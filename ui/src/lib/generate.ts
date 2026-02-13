import JSZip from "jszip";
import {
  create_payload,
  WasmPayload,
  init_logger,
  flush_logs,
} from "../../../wasm/pkg";
import { toast } from "sonner";

export async function generate(
  mac: string,
  date: string,
  version: string,
  bundleHackMiiInstaller: boolean,
  log_cb: (msg: string) => void,
): Promise<boolean> {
  let payload: null | WasmPayload = null;
  try {
    init_logger();
    payload = create_payload(mac, date, version);
    flush_logs(log_cb);
    const bin = new Uint8Array(payload.bin);
    const path = payload.path;
    const fileName = payload.file_name;

    const zipNameWithoutExt = `${mac.replaceAll("-", "")}-${date.replaceAll("-", "")}-${version}`;

    const zip = await createZip(
      zipNameWithoutExt,
      bin,
      path,
      fileName,
      bundleHackMiiInstaller,
      log_cb,
    );

    log_cb("[INFO] initiating zip download");
    const blob = await zip.generateAsync({ type: "blob" });
    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = "output.zip";
    document.body.appendChild(link);
    link.click();

    URL.revokeObjectURL(link.href);
    document.body.removeChild(link);
    return true;
  } catch (e) {
    if (typeof e === "string") {
      toast.error(`Error: ${e}`);
    }
    console.error(e);
    log_cb(`[ERROR] ${e}`);
    return false;
  } finally {
    if (payload !== null) {
      console.log("called free");
      payload.free();
    }
  }
}

export async function createZip(
  rootFolderName: string,
  binary: Uint8Array,
  path: string,
  fileName: string,
  bundleHackMiiInstaller: boolean,
  log_cb: (msg: string) => void,
): Promise<JSZip> {
  try {
    const zip = new JSZip();
    log_cb("[INFO] creating zip...");
    zip.file(`${rootFolderName}/${path}/${fileName}`, binary, {
      binary: true,
    });

    if (bundleHackMiiInstaller) {
      log_cb("[INFO] adding HackMii files...");
      const req = await fetch("/hackmii_installer_v1.2.zip");
      const buf = await req.arrayBuffer();
      const hackMiiZip = await JSZip.loadAsync(buf);

      const files = Object.values(hackMiiZip.files)
        .filter((fileOrDir) => fileOrDir.dir === false)
        .map((file) => ({
          name: file.name.replace("hackmii_installer_v1.2/", ""),
          original: file,
        }));

      for (const { name, original } of files) {
        zip.file(`${rootFolderName}/${name}`, original.async("arraybuffer"), {
          binary: true,
        });
      }
    }

    return zip;
  } catch (e) {
    throw `Failed to create zip: ${e}`;
  }
}
