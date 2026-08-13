import { ref } from "vue";

import { Parameter } from "@/core/models/parameter";
import { CommandResult } from "@/core/models/dto/command_result";

import { getParameters, updateParameters } from "@/core/services/parameter_service";

export function useParameters() {
  const parameters = ref<Parameter[]>();
  const loading = ref(false);

  async function loadParameters(): Promise<string | null> {
    if (loading.value) return null;
    loading.value = true;
    let err: string | null = null;
    try {
      parameters.value = await getParameters();
    } catch (error) {
      err = error as string;
    } finally {
      loading.value = false;
    }
    return err;
  }

  async function saveParameters(parameters: Parameter[]): Promise<CommandResult> {
    return await updateParameters(parameters);
  }

  return { parameters, loading, loadParameters, saveParameters };
}
