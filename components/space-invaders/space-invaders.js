import init, {Engine, Result, Action} from '../../package/space_invaders.js';
String.prototype.replace('&nbsp',' ');

await init();
const engine = Engine.new();

export default class SpaceInvaders extends AbstractComponent {
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
        this.keyBind();

        // Run the animation loop.
        let animate = () => {
            if (engine.is_endgame() != Result.NONE) {
                window.removeEventListener('keyup', keyup);
                return;
            }
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

    keyBind() {
        const keyup = (event) => {
            if (event.code === 'ArrowLeft') {
                engine.input(Action.LEFT);
            }
            if (event.code === 'ArrowRight') {
                engine.input(Action.RIGHT);
            }
            if (event.code === 'Space') {
                engine.input(Action.SHOOT);
            }
        };

        window.addEventListener('keyup', keyup);
    }
    
    render() {
        this.#_span.innerText = engine.render();
    }

    tick() {
        const now = performance.now();
        while (this.#_tick.length > 0 && this.#_tick[0] <= now - 1000) {
            this.#_tick.shift();
        }
        this.#_tick.push(now);
        engine.tick();
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