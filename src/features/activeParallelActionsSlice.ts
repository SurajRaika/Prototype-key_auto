import { createSlice, PayloadAction } from '@reduxjs/toolkit';
import { Action } from "../types";





const initialState: Action[] = [];
export const activeParallelActionsSlice = createSlice({
  name: 'activeParallelActions',
  initialState,
  reducers: {
    setDisabled: (state) => {
      state.length = 0; // Clear the array
    },

    // Add action reducers here if needed
  },
});

export const {  } = activeParallelActionsSlice.actions;

export default activeParallelActionsSlice.reducer;