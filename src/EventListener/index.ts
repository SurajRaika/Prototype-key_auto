// import React from 'react';
// import { useAppSelector, useAppDispatch } from '../hooks';
import { increment} from '../features/eventSlice';
import type {AppDispatch } from '../store/EventStore';

function ListenGlobalEvent(dispatch:AppDispatch) {
    console.log("Initialize the global event listener => Starting");
    
    setInterval(()=>{
        dispatch(increment());
        // console.log('Incremented',count);
    },1000)
    console.log("Initialize the global event listener => Completed");
    
};


export default ListenGlobalEvent;