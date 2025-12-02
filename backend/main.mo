import OrderedMap "mo:base/OrderedMap";
import Text "mo:base/Text";
import Iter "mo:base/Iter";

actor VoileSimulator {

  transient let textMap = OrderedMap.Make<Text>(Text.compare);

  var stepTemplates = textMap.fromIter<Text>(
    Iter.fromArray([
      ("step1", "Generate Private Exit Note: Create an encrypted note with hidden amount, timing, and wallet ID."),
      ("step2", "Create Zero-Knowledge Proof: Generate a cryptographic proof validating the exit request without revealing private data."),
      ("step3", "Submit to LP Network: Submit proof to liquidity providers for validation."),
      ("step4", "LP Validation: Mock LP interface validates proof authenticity without accessing sensitive user data."),
      ("step5", "Private Transfer: Trigger scripted liquidity transfer upon successful proof validation."),
      ("step6", "Settlement: Consume original note and repay LP, completing the private exit."),
    ])
  );

  var tooltips = textMap.fromIter<Text>(
    Iter.fromArray([
      ("encryptedNotes", "Shows only cryptographic commitments and hashes. Actual amounts, timing, and wallet IDs remain hidden."),
      ("zkProofs", "Zero-knowledge proofs validate transactions without revealing sensitive data."),
      ("lpInterface", "Liquidity providers can verify proof authenticity without accessing user details."),
      ("settlement", "Final settlement process ensures privacy and security for both users and LPs."),
    ])
  );

  var helpTexts = textMap.fromIter<Text>(
    Iter.fromArray([
      ("privacy", "Your transaction details remain private. Only cryptographic commitments are visible."),
      ("progress", "Follow the step-by-step process to understand how private exit liquidity works."),
      ("reset", "You can restart the simulation at any time with new parameters."),
    ])
  );

  public query func getStepTemplate(stepId : Text) : async ?Text {
    textMap.get(stepTemplates, stepId);
  };

  public query func getTooltip(tooltipId : Text) : async ?Text {
    textMap.get(tooltips, tooltipId);
  };

  public query func getHelpText(helpId : Text) : async ?Text {
    textMap.get(helpTexts, helpId);
  };

  public query func getAllStepTemplates() : async [(Text, Text)] {
    Iter.toArray(textMap.entries(stepTemplates));
  };

  public query func getAllTooltips() : async [(Text, Text)] {
    Iter.toArray(textMap.entries(tooltips));
  };

  public query func getAllHelpTexts() : async [(Text, Text)] {
    Iter.toArray(textMap.entries(helpTexts));
  };
};
