class FrameRate extends AbstractComponent {
    #_amount;
    #_identifier;
    #_unsubscribe;

    constructor() {
        super();
    }

    static get observedAttributes() {
        return ['amount', 'identifier'];
    }

    get amount() {
        return this.#_amount;
    }

    set amount(value) {
        let amount = this.getElementsByTagName('span');
        if (amount.length == 0) {
            let span = document.createElement('span');
            span['slot'] = 'amount';
            this.appendChild(span);
            amount = span;
        } else {
            amount = amount[0];
        }
        amount.innerText = value;
        this.#_amount = value;
    }

    get identifier() {
        return this.#_identifier;
    }

    set identifier(value) {
        this.#_identifier = value;

        this.#_unsubscribe = ComponentEvent.subscribe(this.#_identifier, (data) => {
            if (typeof(data) !== ' number') {
                throw new Error(`Unable to process the following '${data}'`);
            }

            this.amount = data;
        });
    }

    disconnectedCallback() {
        this.#_unsubscribe();
    }
}