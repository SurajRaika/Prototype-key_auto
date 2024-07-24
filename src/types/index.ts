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