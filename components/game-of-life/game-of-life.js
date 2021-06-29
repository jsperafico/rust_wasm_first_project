import init, {Universe} from '../../package/game_of_life.js';

await init();
const universe = Universe.new();

export default class GameOfLife extends AbstractComponent {
    #_identifier;
    #_tick;
    #_span;

    constructor() {
        super();

        let span = document.createElement('span');
        span['slot'] = 'canvas';
        this.appendChild(span);
        this.#_span = span;
        this.#_tick = [];
    }

    connectedCallback() {
        // Run the animation loop.
        let animate = () => {
            requestAnimationFrame(animate);

            this.tick();
            ComponentEvent.publish(this.#_identifier, this.#_tick.length);
            this.render();
        }
        animate();
    }

    attributeChangedCallback(attr, oldValue, newValue) {
        super.attributeChangedCallback(attr, oldValue, newValue);
    }
    
    disconnectedCallback() {
    }

    render() {
        this.#_span.innerText = universe.render();
    }

    tick() {
        const now = performance.now();
        while (this.#_tick.length > 0 && this.#_tick[0] <= now - 1000) {
            this.#_tick.shift();
        }
        this.#_tick.push(now);
        universe.tick();
    }

    static get observedAttributes() {
        return ['identifier'];
    }
    
    get identifier() {
        return this.#_identifier;
    }

    set identifier(value) {
        this.#_identifier = value;
    }
}