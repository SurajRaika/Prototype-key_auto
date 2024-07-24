import React from 'react';
import { useAppSelector, useAppDispatch } from '../hooks';
import { increment, decrement } from '../features/eventSlice';

function Counter() {
  const count = useAppSelector((state) => state.event.value);
  const dispatch = useAppDispatch();

  return (
    <div>
      <div>Count: {count}</div>
      <button onClick={() => dispatch(increment())}>Increment</button>
      <button onClick={() => dispatch(decrement())}>Decrement</button>
    </div>
  );
}

export default Counter;