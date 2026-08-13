import { ref } from "vue";

import { SystemStatus } from "@/core/models/system_status";
import { CommandResult } from "@/core/models/dto/command_result";

import { getSystemStatus, updateSystemStatus } from "@/core/services/system_status_service";

export function useSystemStatus() {
  const systemStatus = ref<SystemStatus[]>();
  const loading = ref(false);

  async function loadSystemStatus(): Promise<string | null> {
    if (loading.value) return null;
    loading.value = true;
    let err: string | null = null;
    try {
      systemStatus.value = await getSystemStatus();
    } catch (error) {
      err = error as string;
    } finally {
      loading.value = false;
    }
    return err;
  }

  async function saveSystemStatus(systemStatus: SystemStatus[]): Promise<CommandResult> {
    return await updateSystemStatus(systemStatus);
  }

  return { systemStatus, loading, loadSystemStatus, saveSystemStatus };
}
