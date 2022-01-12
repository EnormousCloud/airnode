import { Footer } from '../components/Footer';
import { TextInput } from '../components/forms/TextInput';
import { Button } from '../components/forms/Button';

interface AddContractProps {
    // dataStatus: DataStatus
    onSubmit: Function
}

export const AddContract = (props: AddContractProps) => {
    const nop = () => {}
    return (
        <div>
            <main>
                <div className="inner" style={{ minHeight: 'auto' }}>
                    <div className="content">
                        <h1>Add RRP Contract</h1>
                        
                        <form autoComplete="on" spellCheck="false">
                            <div style={{ textAlign: 'center' }}>
                                <div className="dash-row">
                                    <div className="dash-col-100">
                                        <label>
                                            <h3 className="cell-title"> JSON+RPC URL*:</h3>
                                            <TextInput name="jsonrpc" placeholder="Network RPC URL" value='' onChange={nop} />
                                        </label>
                                    </div>
                                </div>
                                <div className="dash-row">
                                    <div className="dash-col-100">
                                        <label>
                                            <h3 className="cell-title">Contract Address*: </h3>
                                            <TextInput name="contract" placeholder=""  value='' onChange={nop} />
                                        </label>
                                    </div>
                                </div>
                                <div className="dash-row">
                                    <div style={{ width: 120 }} className="cell dash-col-3">
                                        <h3 className="cell-title">Min Block*:</h3>
                                        <TextInput name="min_block" placeholder="0" style={{ width: 120, textAlign: "center" }} value='' onChange={nop}   />
                                    </div>
                                    <div style={{ width: 120 }} className="cell dash-col-3">
                                        <h3 className="cell-title">Max Block:</h3>
                                        <TextInput name="max_block" placeholder="HEAD" style={{ width: 120, textAlign: "center" }} value='' onChange={nop}   />
                                    </div>
                                    <div style={{ width: 120 }} className="cell dash-col-3">
                                        <h3 className="cell-title">Batch Size*:</h3>
                                        <TextInput name="batch_size" placeholder="" style={{ width: 120, textAlign: "center" }}  value='' onChange={nop} />
                                    </div>
                                </div>

                                <div className="dash-row">
                                    <div style={{ width: 200, margin: '20px auto' }}>
                                        <Button type="primary" onClick={() => { props.onSubmit(); }} label='Start Syncing' />
                                    </div>
                                </div>
                            </div>
                        </form>
                    </div>
                </div>
            </main>
            <Footer />
        </div>
    )
};