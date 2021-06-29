export default class FrameRate extends AbstractComponent {
    #_amount;
    #_identifier;
    #_unsubscribe;
    #_span;

    constructor() {
        super();

        let span = document.createElement('span');
        span['slot'] = 'amount';
        this.appendChild(span);
        this.#_span = span;
    }

    connectedCallback() {
        this.#_unsubscribe = ComponentEvent.subscribe(this.#_identifier, (data) => {
            if (typeof(data) !== "number") {
                throw new Error(`Unable to process the following '${data}'`);
            }
            this.setAttribute('amount', data);
        });
    }

    attributeChangedCallback(attr, oldValue, newValue) {
        super.attributeChangedCallback(attr, oldValue, newValue);
        if (attr == 'amount') {
            this.render();
        }
    }

    render() {
        this.#_span.innerText = this.amount;
    }
    
    static get observedAttributes() {
        return ['amount', 'identifier'];
    }

    get amount() {
        return this.#_amount;
    }

    set amount(value) {
        this.#_amount = value;
    }

    get identifier() {
        return this.#_identifier;
    }

    set identifier(value) {
        this.#_identifier = value;
    }

    disconnectedCallback() {
        this.#_unsubscribe();
    }
}