import { invoke } from "@tauri-apps/api/core";
import { SystemStatus } from "@/core//models/system_status";
import { CommandResult } from "@/core/models/dto/command_result";

export async function getSystemStatus(): Promise<SystemStatus[]> {
  return await invoke<SystemStatus[]>("get_system_status");
}

export async function updateSystemStatus(
  systemStatus: SystemStatus[],
): Promise<CommandResult> {
  return await invoke<CommandResult>("update_system_status", { systemStatus });
}
