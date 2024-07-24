import { configureStore } from '@reduxjs/toolkit';
import eventReducer from '../features/eventSlice';
import activeParallelActionsSlice from '../features/activeParallelActionsSlice';



// todo add activeParallelActionsSlice 
export const store = configureStore({
  reducer: {
    // We'll add reducers here
    event:eventReducer,
    activeParallelActions: activeParallelActionsSlice

  },
});

export type RootState = ReturnType<typeof store.getState>;
export type AppDispatch = typeof store.dispatch;