import React from 'react';
import { useAppSelector, useAppDispatch } from '../../hooks';
import { increment, decrement } from '../../features/eventSlice';

function NewAction() {
  const count = useAppSelector((state) => state.event.value);
  const dispatch = useAppDispatch();

  return (
    <div>
     Action
    </div>
  );
}

export default NewAction;