class FrameRate extends AbstractComponent {
    #_amount;
    #_identifier;
    #_unsubscribe;

    constructor() {
        super();
    }

    connectedCallback() {
        super.connectedCallback();

        this.#_unsubscribe = ComponentEvent.subscribe(this.#_identifier, (data) => {
            if (typeof(data) !== ' number') {
                throw new Error(`Unable to process the following '${data}'`);
            }

            this.amount = data;
        });
    }

    attributeChangedCallback(attr, oldValue, newValue) {
        super.attributeChangedCallback(attr, oldValue, newValue);
        if (attr == 'amount') {
            this.render();
        }
    }

    render() {
        let amount = this.getElementsByTagName('span');
        if (amount.length == 0) {
            let span = document.createElement('span');
            span['slot'] = 'amount';
            this.appendChild(span);
            amount = span;
        } else {
            amount = amount[0];
        }
        amount.innerText = this.amount;
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