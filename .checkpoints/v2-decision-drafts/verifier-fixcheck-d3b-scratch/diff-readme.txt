diff --git a/docs/adr/README.md b/docs/adr/README.md
index 887d675..554120b 100644
--- a/docs/adr/README.md
+++ b/docs/adr/README.md
@@ -40,4 +40,4 @@ by an agent, are in the drafts and in [SPIKE_PLAN](../SPIKE_PLAN.md), not here.
-| WEAPON-UNLOCKS (OQ-0005-AB5) | How does an account-unlocked weapon reach a match (LOOP_V2 §8, VISION_V2 §1), when buying offers no weapon choice and other weapons come only from drops? | Mustafa, with the Advisor (LOOP_V2 §8 is theirs) | [ADR 0005](0005-content-abilities-assets.md) line 1239 |
-| UPGRADE-SCOPE: carry slots (OQ-0005-AB1) | How many weapons can a player carry besides the class weapon, and what happens to the one in hand on a pickup? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1186 |
-| UPGRADE-SCOPE: looted ammo (OQ-0005-AB2) | What does a looted gun come with, and can it be refilled? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1198 |
-| UPGRADE-SCOPE: levelling (OQ-0005-AB3) | What earns a level, how fast, and does a track the player switches to catch up? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1206 |
+| WEAPON-UNLOCKS (OQ-0005-AB5) | How does an account-unlocked weapon reach a match (LOOP_V2 §8, VISION_V2 §1), when buying offers no weapon choice and other weapons come only from drops? | Mustafa, with the Advisor (LOOP_V2 §8 is theirs) | [ADR 0005](0005-content-abilities-assets.md) line 1241 |
+| UPGRADE-SCOPE: carry slots (OQ-0005-AB1) | How many weapons can a player carry besides the class weapon, and what happens to the one in hand on a pickup? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1188 |
+| UPGRADE-SCOPE: looted ammo (OQ-0005-AB2) | What does a looted gun come with, and can it be refilled? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1200 |
+| UPGRADE-SCOPE: levelling (OQ-0005-AB3) | What earns a level, how fast, and does a track the player switches to catch up? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1208 |
@@ -96,19 +96,19 @@ by an agent, are in the drafts and in [SPIKE_PLAN](../SPIKE_PLAN.md), not here.
-| OQ-0005-AB4 | What is one track: which definitions share one? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1226 |
-| OQ-0005-AB6 | How do the player's tracks sit beside the team's tech tiers? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1255 |
-| OQ-0005-AB7 | May a pick add raw power (more damage, more health), or only change how a unit or weapon plays? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1263 |
-| OQ-0005-AB8 | The offer: when does it appear, and when can the player answer it? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1274 |
-| OQ-0005-AB9 | Which drop and pickup rules does UPGRADE-SCOPE leave unstated? | Mustafa, with the game-designer | [ADR 0005](0005-content-abilities-assets.md) line 1288 |
-| OQ-0005-AB10 | Do bots have tracks and make picks, and what does a human get when taking a bot's slot? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1305 |
-| OQ-0005-AB11 | Does the Riot Shield answer the Banzai blade, and is the Peacekeeper's side of the triangle strong enough before Tech 1? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1319 |
-| OQ-0005-AB12 | Is tranek's write-up accepted as the GAS reference, or does a verifier read Epic's documentation first? | the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 1330 |
-| OQ-0005-AB13 | When does scripting (option A3) come back? | the Advisor (the source) | [ADR 0005](0005-content-abilities-assets.md) line 1339 |
-| OQ-0005-AB14 | What does spawn protection cover, and what ends it early? | Mustafa (the 3 s is the Advisor's `[DEFAULT]`; what it covers and what ends it are new rules) | [ADR 0005](0005-content-abilities-assets.md) line 1347 |
-| OQ-0005-AB15 | Who sees a player's upgrade picks? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1365 |
-| OQ-0005-AP1 | Where does the automated asset step run, what does it write, and where do the model files live (which decides who can see unreleased art)? | the asset-pipeline research, then the asset spike; the files' visibility follows Mustafa's answer to OQ-0006-18, due before the first authored model is committed | [ADR 0005](0005-content-abilities-assets.md) line 1632 |
-| OQ-0005-AP2 | Which asset families become authored models, and which stay procedural builders? | Mustafa, in the look proof test (how each looks, beside legacy screenshots, LOOK_V2 §9), with the asset-pipeline research's cost and tooling. Trees wait for the tree style (LOOK_V2 §8 `[OPEN]`) | [ADR 0005](0005-content-abilities-assets.md) line 1646 |
-| OQ-0005-AP3 | May an AI-generated draft model ever ship? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1654 |
-| OQ-0005-AS2 | Which tool builds the LOD levels, and how many levels? | the asset-pipeline research, then the asset spike and Mustafa (silhouettes kept) | [ADR 0005](0005-content-abilities-assets.md) line 1809 |
-| OQ-0005-1 | Do presentation references sit on the simulation definitions, or in client definitions keyed by the same id? | the Advisor, at the stop review (the game-designer changes AB-4's kinds if the answer needs it) | [ADR 0005](0005-content-abilities-assets.md) line 2037 |
-| OQ-0005-2 | Is a weapon definition the same thing as a weapon type? | Mustafa answers OQ-0005-AB4; the question stays open for the Advisor until then | [ADR 0005](0005-content-abilities-assets.md) line 2057 |
-| OQ-0005-3 | What does the content hash cover? | the Advisor, with the infra-engineer, whose ADR 0006 owns the analysis (R6-TEL-REC3) | [ADR 0005](0005-content-abilities-assets.md) line 2068 |
-| OQ-0005-4 | What do the Character Lab and the Anim Lab work on once soldiers are authored models? | Mustafa with the Advisor (EDITOR_V2 is theirs) | [ADR 0005](0005-content-abilities-assets.md) line 2080 |
+| OQ-0005-AB4 | What is one track: which definitions share one? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1228 |
+| OQ-0005-AB6 | How do the player's tracks sit beside the team's tech tiers? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1257 |
+| OQ-0005-AB7 | May a pick add raw power (more damage, more health), or only change how a unit or weapon plays? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1265 |
+| OQ-0005-AB8 | The offer: when does it appear, and when can the player answer it? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1276 |
+| OQ-0005-AB9 | Which drop and pickup rules does UPGRADE-SCOPE leave unstated? | Mustafa, with the game-designer | [ADR 0005](0005-content-abilities-assets.md) line 1290 |
+| OQ-0005-AB10 | Do bots have tracks and make picks, and what does a human get when taking a bot's slot? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1307 |
+| OQ-0005-AB11 | Does the Riot Shield answer the Banzai blade, and is the Peacekeeper's side of the triangle strong enough before Tech 1? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1321 |
+| OQ-0005-AB12 | Is tranek's write-up accepted as the GAS reference, or does a verifier read Epic's documentation first? | the Advisor | [ADR 0005](0005-content-abilities-assets.md) line 1332 |
+| OQ-0005-AB13 | When does scripting (option A3) come back? | the Advisor (the source) | [ADR 0005](0005-content-abilities-assets.md) line 1341 |
+| OQ-0005-AB14 | What does spawn protection cover, and what ends it early? | Mustafa (the 3 s is the Advisor's `[DEFAULT]`; what it covers and what ends it are new rules) | [ADR 0005](0005-content-abilities-assets.md) line 1349 |
+| OQ-0005-AB15 | Who sees a player's upgrade picks? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1367 |
+| OQ-0005-AP1 | Where does the automated asset step run, what does it write, and where do the model files live (which decides who can see unreleased art)? | the asset-pipeline research, then the asset spike; the files' visibility follows Mustafa's answer to OQ-0006-18, due before the first authored model is committed | [ADR 0005](0005-content-abilities-assets.md) line 1634 |
+| OQ-0005-AP2 | Which asset families become authored models, and which stay procedural builders? | Mustafa, in the look proof test (how each looks, beside legacy screenshots, LOOK_V2 §9), with the asset-pipeline research's cost and tooling. Trees wait for the tree style (LOOK_V2 §8 `[OPEN]`) | [ADR 0005](0005-content-abilities-assets.md) line 1648 |
+| OQ-0005-AP3 | May an AI-generated draft model ever ship? | Mustafa | [ADR 0005](0005-content-abilities-assets.md) line 1656 |
+| OQ-0005-AS2 | Which tool builds the LOD levels, and how many levels? | the asset-pipeline research, then the asset spike and Mustafa (silhouettes kept) | [ADR 0005](0005-content-abilities-assets.md) line 1811 |
+| OQ-0005-1 | Do presentation references sit on the simulation definitions, or in client definitions keyed by the same id? | the Advisor, at the stop review (the game-designer changes AB-4's kinds if the answer needs it) | [ADR 0005](0005-content-abilities-assets.md) line 2039 |
+| OQ-0005-2 | Is a weapon definition the same thing as a weapon type? | Mustafa answers OQ-0005-AB4; the question stays open for the Advisor until then | [ADR 0005](0005-content-abilities-assets.md) line 2059 |
+| OQ-0005-3 | What does the content hash cover? | the Advisor, with the infra-engineer, whose ADR 0006 owns the analysis (R6-TEL-REC3) | [ADR 0005](0005-content-abilities-assets.md) line 2070 |
+| OQ-0005-4 | What do the Character Lab and the Anim Lab work on once soldiers are authored models? | Mustafa with the Advisor (EDITOR_V2 is theirs) | [ADR 0005](0005-content-abilities-assets.md) line 2082 |
