import { useSignAndExecuteTransaction, useSuiClient } from '@mysten/dapp-kit';
import { Transaction } from '@mysten/sui/transactions';
import { useState } from 'react';

export function usePayment() {
  const { mutateAsync: signAndExecute } = useSignAndExecuteTransaction();
  const suiClient = useSuiClient();
  const [isProcessing, setIsProcessing] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const payForTemplate = async (
    recipientAddress: string,
    amountMist: number
  ): Promise<{ digest: string; coinObjectId: string }> => {
    setIsProcessing(true);
    setError(null);

    try {
      // Create a new transaction
      const tx = new Transaction();

      // Split a coin for the exact payment amount
      const [coin] = tx.splitCoins(tx.gas, [amountMist]);

      // Transfer the split coin to the recipient (template creator)
      tx.transferObjects([coin], recipientAddress);

      // Sign and execute the transaction
      const result = await signAndExecute({
        transaction: tx,
      });

      // Wait for the transaction to be confirmed
      const txResponse = await suiClient.waitForTransaction({
        digest: result.digest,
        options: {
          showEffects: true,
          showObjectChanges: true,
        },
      });

      // Extract the coin object ID from the transaction effects
      const createdObjects = txResponse.effects?.created || [];
      const coinObjectId = createdObjects[0]?.reference?.objectId || result.digest;

      setIsProcessing(false);
      return {
        digest: result.digest,
        coinObjectId,
      };
    } catch (err: any) {
      const errorMessage = err.message || 'Payment failed';
      setError(errorMessage);
      setIsProcessing(false);
      throw new Error(errorMessage);
    }
  };

  return {
    payForTemplate,
    isProcessing,
    error,
  };
}
