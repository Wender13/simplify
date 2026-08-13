export interface SystemStatusInterface {
  id_system_status: number;
  name: string;
  value: string;
}

export class SystemStatus {
  constructor(systemStatus: SystemStatusInterface) {
    this.id_system_status = systemStatus.id_system_status;
    this.name = systemStatus.name;
    this.value = systemStatus.value;
  }
  id_system_status: number;
  name: string;
  value: string;
}
