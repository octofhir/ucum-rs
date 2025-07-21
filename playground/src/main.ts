import './app.css';
import App from './App.svelte';
import { mount } from 'svelte';

const target = document.getElementById("app");
if(!target) throw new Error("No target element found")

const app = mount(App, { target });

export default app;
