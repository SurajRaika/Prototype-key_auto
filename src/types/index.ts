export interface Action {
    id: string;
    executionTime: {
      seconds: number;
      nanoseconds: number;
    };
    startTime: {
      seconds: number;
      nanoseconds: number;
    };
}

export type ParallelActions = Action[]; // Use 'type' instead of 'interface'

export interface EventKey {
  id: number;
  name: string; // Use 'string' instead of 'String'
  description:string;
  event: ParallelActions[]; // Remove '[]'
  actions: ParallelActions[]; // Remove '[]'
}

export type EventKeys = EventKey[]; // Use 'type' instead of 'interface'