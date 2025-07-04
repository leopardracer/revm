use crate::{
    inspect::{InspectCommitEvm, InspectEvm},
    Inspector, InspectorEvmTr, InspectorHandler, JournalExt,
};
use context::{ContextSetters, ContextTr, Evm, JournalTr};
use database_interface::DatabaseCommit;
use handler::{
    instructions::InstructionProvider, EthFrame, EvmTr, EvmTrError, Handler, MainnetHandler,
    PrecompileProvider,
};
use interpreter::{interpreter::EthInterpreter, InterpreterResult};
use state::EvmState;

// Implementing InspectorHandler for MainnetHandler.
impl<EVM, ERROR> InspectorHandler for MainnetHandler<EVM, ERROR, EthFrame<EthInterpreter>>
where
    EVM: InspectorEvmTr<
        Context: ContextTr<Journal: JournalTr<State = EvmState>>,
        Frame = EthFrame<EthInterpreter>,
        Inspector: Inspector<<<Self as Handler>::Evm as EvmTr>::Context, EthInterpreter>,
    >,
    ERROR: EvmTrError<EVM>,
{
    type IT = EthInterpreter;
}

// Implementing InspectEvm for Evm
impl<CTX, INSP, INST, PRECOMPILES> InspectEvm
    for Evm<CTX, INSP, INST, PRECOMPILES, EthFrame<EthInterpreter>>
where
    CTX: ContextSetters + ContextTr<Journal: JournalTr<State = EvmState> + JournalExt>,
    INSP: Inspector<CTX, EthInterpreter>,
    INST: InstructionProvider<Context = CTX, InterpreterTypes = EthInterpreter>,
    PRECOMPILES: PrecompileProvider<CTX, Output = InterpreterResult>,
{
    type Inspector = INSP;

    fn set_inspector(&mut self, inspector: Self::Inspector) {
        self.inspector = inspector;
    }

    fn inspect_one_tx(&mut self, tx: Self::Tx) -> Result<Self::ExecutionResult, Self::Error> {
        self.set_tx(tx);
        MainnetHandler::default().inspect_run(self)
    }
}

// Implementing InspectCommitEvm for Evm
impl<CTX, INSP, INST, PRECOMPILES> InspectCommitEvm
    for Evm<CTX, INSP, INST, PRECOMPILES, EthFrame<EthInterpreter>>
where
    CTX: ContextSetters
        + ContextTr<Journal: JournalTr<State = EvmState> + JournalExt, Db: DatabaseCommit>,
    INSP: Inspector<CTX, EthInterpreter>,
    INST: InstructionProvider<Context = CTX, InterpreterTypes = EthInterpreter>,
    PRECOMPILES: PrecompileProvider<CTX, Output = InterpreterResult>,
{
}

// Implementing InspectorEvmTr for Evm
impl<CTX, INSP, I, P> InspectorEvmTr for Evm<CTX, INSP, I, P, EthFrame<EthInterpreter>>
where
    CTX: ContextTr<Journal: JournalExt> + ContextSetters,
    I: InstructionProvider<Context = CTX, InterpreterTypes = EthInterpreter>,
    P: PrecompileProvider<CTX, Output = InterpreterResult>,
    INSP: Inspector<CTX, I::InterpreterTypes>,
{
    type Inspector = INSP;

    fn inspector(&mut self) -> &mut Self::Inspector {
        &mut self.inspector
    }

    fn ctx_inspector(&mut self) -> (&mut Self::Context, &mut Self::Inspector) {
        (&mut self.ctx, &mut self.inspector)
    }

    fn ctx_inspector_frame(
        &mut self,
    ) -> (&mut Self::Context, &mut Self::Inspector, &mut Self::Frame) {
        (&mut self.ctx, &mut self.inspector, self.frame_stack.get())
    }

    fn ctx_inspector_frame_instructions(
        &mut self,
    ) -> (
        &mut Self::Context,
        &mut Self::Inspector,
        &mut Self::Frame,
        &mut Self::Instructions,
    ) {
        (
            &mut self.ctx,
            &mut self.inspector,
            self.frame_stack.get(),
            &mut self.instruction,
        )
    }
}
