import { invoke } from "@tauri-apps/api/core";
import { Parameter } from "@/core//models/parameter";
import { CommandResult } from "@/core/models/dto/command_result";

export async function getParameters(): Promise<Parameter[]> {
  return await invoke<Parameter[]>("get_parameters");
}

export async function updateParameters(
  parameters: Parameter[],
): Promise<CommandResult> {
  return await invoke<CommandResult>("update_parameters", { parameters });
}
