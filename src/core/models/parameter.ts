export interface ParameterInterface {
  id_parameter: number;
  name: string;
  value: string;
}

export class Parameter {
  constructor(parameter: ParameterInterface) {
    this.id_parameter = parameter.id_parameter;
    this.name = parameter.name;
    this.value = parameter.value;
  }
  id_parameter: number;
  name: string;
  value: string;
}
